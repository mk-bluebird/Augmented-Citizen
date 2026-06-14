// filename: NeuroRightsController.java
// destination: java-neuro-rights-service/src/main/java/org/augcit/api/NeuroRightsController.java
// SPDX-License-Identifier: MIT OR Apache-2.0

package org.augcit.api;

import org.augcit.policy.PolicyEvaluationEngine;
import org.augcit.policy.model.Decision;
import org.augcit.host.HostProfile;
import org.augcit.aln.model.ChannelCategory;
import jakarta.ws.rs.*;
import jakarta.ws.rs.core.MediaType;
import jakarta.ws.rs.core.Response;
import jakarta.inject.Inject;

@Path("/v1/neurorights")
@Produces(MediaType.APPLICATION_JSON)
@Consumes(MediaType.APPLICATION_JSON)
public class NeuroRightsController {

    @Inject PolicyEvaluationEngine policyEngine;
    @Inject HostProfileResolver hostResolver;

    @POST
    @Path("/evaluate/read")
    public Response evaluateRead(@HeaderParam("X-Host-DID") String hostDid, 
                                 @QueryParam("channel") ChannelCategory channel,
                                 @QueryParam("purpose") String purpose) {
        
        HostProfile host = hostResolver.resolveOrThrow(hostDid);
        Decision decision = policyEngine.checkNeurodataRead(host, channel, purpose);

        return mapDecisionToResponse(decision);
    }

    @POST
    @Path("/evaluate/modulation")
    public Response evaluateModulation(@HeaderParam("X-Host-DID") String hostDid,
                                       @QueryParam("channel") ChannelCategory channel,
                                       ModulationRequest request) {
        
        HostProfile host = hostResolver.resolveOrThrow(hostDid);
        Decision decision = policyEngine.checkTherapeuticModulation(host, channel, request.toParams());

        return mapDecisionToResponse(decision);
    }

    private Response mapDecisionToResponse(Decision decision) {
        return switch (decision) {
            case Decision.Allow a -> Response.ok(a).build();
            case Decision.Deny d -> Response.status(Response.Status.FORBIDDEN).entity(d).build();
            case Decision.RequireConfirmation rc -> Response.status(Response.Status.ACCEPTED).entity(rc).build();
        };
    }
}
