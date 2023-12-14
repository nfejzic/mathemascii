use alemat::elements::Ident;

use crate::lexer::token::TokenKind;

use super::macros::generate_impl;

generate_impl!(
    Greek,
    Greeks,
    "alpha" => Alpha,
    "beta" => Beta,
    "gamma" => Gamma,
    "Gamma" => BigGamma,
    "delta" => Delta,
    "epsilon" => Epsilon,
    "varepsilon" => Varepsilon,
    "zeta" => Zeta,
    "eta" => Eta,
    "theta" => Theta,
    "vartheta" => Vartheta,
    "iota" => Iota,
    "kappa" => Kappa,
    "lambda" => Lambda,
    "mu" => Mu,
    "nu" => Nu,
    "xi" => Xi,
    "Xi" => BigXi,
    "pi" => Pi,
    "Pi" => BigPi,
    "rho" => Rho,
    "sigma" => Sigma,
    "Sigma" => BigSigma,
    "tau" => Tau,
    "upsilon" => Upsilon,
    "phi" => Phi,
    "Phi" => BigPhi,
    "varphi" => Varphi,
    "chi" => Chi,
    "psi" => Psi,
    "Psi" => BigPsi,
    "omega" => Omega,
    "Omega" => BigOmega
);

impl From<Greek> for TokenKind {
    fn from(value: Greek) -> Self {
        TokenKind::Greek(value)
    }
}

impl From<Greek> for Ident {
    fn from(value: Greek) -> Self {
        match value {
            Greek::Alpha => Ident::alpha(),
            Greek::Beta => Ident::beta(),
            Greek::Gamma => Ident::gamma(),
            Greek::BigGamma => Ident::big_gamma(),
            Greek::Delta => Ident::delta(),
            Greek::Epsilon => Ident::epsilon(),
            Greek::Varepsilon => Ident::varepsilon(),
            Greek::Zeta => Ident::zeta(),
            Greek::Eta => Ident::eta(),
            Greek::Theta => Ident::theta(),
            Greek::Vartheta => Ident::vartheta(),
            Greek::Iota => Ident::iota(),
            Greek::Kappa => Ident::kappa(),
            Greek::Lambda => Ident::lambda(),
            Greek::Mu => Ident::mu(),
            Greek::Nu => Ident::nu(),
            Greek::Xi => Ident::xi(),
            Greek::BigXi => Ident::big_xi(),
            Greek::Pi => Ident::pi(),
            Greek::BigPi => Ident::big_pi(),
            Greek::Rho => Ident::rho(),
            Greek::Sigma => Ident::sigma(),
            Greek::BigSigma => Ident::big_sigma(),
            Greek::Tau => Ident::tau(),
            Greek::Upsilon => Ident::upsilon(),
            Greek::Phi => Ident::phi(),
            Greek::BigPhi => Ident::big_phi(),
            Greek::Varphi => Ident::varphi(),
            Greek::Chi => Ident::chi(),
            Greek::Psi => Ident::psi(),
            Greek::BigPsi => Ident::big_psi(),
            Greek::Omega => Ident::omega(),
            Greek::BigOmega => Ident::big_omega(),
        }
    }
}
