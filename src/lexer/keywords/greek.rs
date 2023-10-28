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
