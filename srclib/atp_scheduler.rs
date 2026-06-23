// srclib/atp_scheduler.rs

#[derive(Debug, Clone)]
pub struct AtpTokenBudgetPerOrgan {
    pub maxenergyj_resp: f32,
    pub maxenergyj_card: f32,
    pub maxenergyj_neuro: f32,
    pub maxenergyj_other: f32,
}

#[derive(Debug, Clone)]
pub struct AtpTokenConsumptionPerOrgan {
    pub resp_used_j: f32,
    pub card_used_j: f32,
    pub neuro_used_j: f32,
    pub other_used_j: f32,
}

impl AtpTokenConsumptionPerOrgan {
    pub fn new() -> Self {
        Self {
            resp_used_j: 0.0,
            card_used_j: 0.0,
            neuro_used_j: 0.0,
            other_used_j: 0.0,
        }
    }

    pub fn can_spend(&self, budget: &AtpTokenBudgetPerOrgan, delta_resp: f32, delta_card: f32, delta_neuro: f32, delta_other: f32) -> bool {
        let new_resp = self.resp_used_j + delta_resp.max(0.0);
        let new_card = self.card_used_j + delta_card.max(0.0);
        let new_neuro = self.neuro_used_j + delta_neuro.max(0.0);
        let new_other = self.other_used_j + delta_other.max(0.0);

        new_resp <= budget.maxenergyj_resp
            && new_card <= budget.maxenergyj_card
            && new_neuro <= budget.maxenergyj_neuro
            && new_other <= budget.maxenergyj_other
    }

    pub fn spend_checked(&mut self, budget: &AtpTokenBudgetPerOrgan, delta_resp: f32, delta_card: f32, delta_neuro: f32, delta_other: f32) -> bool {
        if self.can_spend(budget, delta_resp, delta_card, delta_neuro, delta_other) {
            self.resp_used_j += delta_resp.max(0.0);
            self.card_used_j += delta_card.max(0.0);
            self.neuro_used_j += delta_neuro.max(0.0);
            self.other_used_j += delta_other.max(0.0);
            true
        } else {
            false
        }
    }
}
