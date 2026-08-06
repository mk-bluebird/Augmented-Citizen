import pandas as pd

from dowhy import CausalModel
from causalnex.network import BayesianNetwork
from causalnex.structure.notears import from_pandas


class AugmentationPsiCausalEngine:
    def __init__(
        self,
        df: pd.DataFrame,
        treatment_col: str = "augmentation_intensity",
        outcome_col: str = "psi_continuity",
        motivation_col: str = "motivation",
        plasticity_col: str = "plasticity",
        baseline_psi_col: str = "baseline_psi",
        covariate_cols=None,
    ):
        if covariate_cols is None:
            covariate_cols = ["age", "sleep_quality", "prior_trauma"]
        self.df = df.copy()
        self.treatment_col = treatment_col
        self.outcome_col = outcome_col
        self.motivation_col = motivation_col
        self.plasticity_col = plasticity_col
        self.baseline_psi_col = baseline_psi_col
        self.covariate_cols = covariate_cols
        self.model = None
        self.identified_estimand = None
        self.estimate = None
        self.refutations = []
        self.bn = None

    def build_causal_model(self) -> None:
        covariates_decl = "; ".join(self.covariate_cols) + ";"
        cov_edges = ""
        for c in self.covariate_cols:
            cov_edges += f"{c} -> {self.treatment_col};\n"
            cov_edges += f"{c} -> {self.outcome_col};\n"

        graph_str = f"""
            digraph {{
                {self.outcome_col};
                {self.treatment_col};
                {self.motivation_col};
                {self.plasticity_col};
                {self.baseline_psi_col};
                {covariates_decl}

                {self.motivation_col} -> {self.treatment_col};
                {self.motivation_col} -> {self.outcome_col};
                {self.plasticity_col} -> {self.treatment_col};
                {self.plasticity_col} -> {self.outcome_col};
                {cov_edges}
                {self.baseline_psi_col} -> {self.outcome_col};
                {self.treatment_col} -> {self.outcome_col};
            }}
        """

        self.model = CausalModel(
            data=self.df,
            treatment=self.treatment_col,
            outcome=self.outcome_col,
            graph=graph_str,
        )

    def identify_effect(self) -> None:
        if self.model is None:
            self.build_causal_model()
        self.identified_estimand = self.model.identify_effect()

    def estimate_effect(self, method_name: str = "backdoor.linear_regression") -> None:
        if self.identified_estimand is None:
            self.identify_effect()
        self.estimate = self.model.estimate_effect(
            self.identified_estimand,
            method_name=method_name,
        )

    def run_refutations(self) -> None:
        if self.estimate is None:
            self.estimate_effect()
        self.refutations = [
            self.model.refute_estimate(
                self.identified_estimand,
                self.estimate,
                method_name="random_common_cause",
            ),
            self.model.refute_estimate(
                self.identified_estimand,
                self.estimate,
                method_name="data_subset_refuter",
            ),
        ]

    def build_bayesian_network(self) -> None:
        self.bn = BayesianNetwork(from_pandas(self.df))
        self.bn.fit_node_states_and_cpds(self.df)

    def psi_distribution_under_intervention(self, treatment_value) -> dict:
        if self.bn is None:
            self.build_bayesian_network()
        self.bn.do_intervention(self.treatment_col, treatment_value)
        return self.bn.get_distribution(self.outcome_col)

    def causal_psi_delta_and_confidence(self) -> tuple[float, float]:
        if self.estimate is None:
            self.estimate_effect()
        if not self.refutations:
            self.run_refutations()

        delta_psi = float(self.estimate.value)

        confidence = 1.0
        for r in self.refutations:
            try:
                if hasattr(r, "p_value") and r.p_value is not None:
                    confidence *= max(0.0, min(1.0, 1.0 - float(r.p_value)))
            except Exception:
                confidence *= 0.8

        confidence = max(0.0, min(1.0, confidence))
        return delta_psi, confidence

    def cyberrank_delta(self, gamma: float = 0.1) -> float:
        delta_psi, confidence = self.causal_psi_delta_and_confidence()
        delta_psi_clipped = max(0.0, delta_psi)
        return gamma * delta_psi_clipped * confidence
