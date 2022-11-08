use my_http_server::RequestClaim;

#[derive(Debug, Clone)]
pub struct RequiredClaims {
    required_claims: Vec<String>,
}

impl RequiredClaims {
    pub fn from_vec(claims: Vec<String>) -> Self {
        Self {
            required_claims: claims,
        }
    }

    fn has_claim(&self, claim_id_to_find: &str) -> bool {
        for client_id in &self.required_claims {
            if client_id == claim_id_to_find {
                return true;
            }
        }

        false
    }

    pub fn authorized_by_claims(
        &self,
        request_ip: &str,
        request_claims: Option<&[RequestClaim]>,
    ) -> bool {
        // No Claims means - we are authorizeds
        if self.required_claims.len() == 0 {
            return true;
        }

        if let Some(request_claims) = request_claims {
            for request_claim in request_claims {
                if !self.has_claim(&request_claim.id) {
                    return false;
                }

                if !request_claim.is_ip_allowed(request_ip) {
                    return false;
                }
            }

            return true;
        }
        false
    }
}
