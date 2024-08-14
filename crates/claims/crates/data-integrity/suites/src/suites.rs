#[cfg(feature = "w3c")]
mod w3c;
#[cfg(feature = "w3c")]
pub use w3c::*;

#[cfg(feature = "dif")]
mod dif;
#[cfg(feature = "dif")]
#[allow(unused_imports)]
pub use dif::*;

#[cfg(any(
    feature = "aleo",
    feature = "ethereum",
    feature = "tezos",
    feature = "solana"
))]
mod unspecified;

#[cfg(any(
    feature = "aleo",
    feature = "ethereum",
    feature = "tezos",
    feature = "solana"
))]
#[allow(unused_imports)]
pub use unspecified::*;

macro_rules! try_from_type {
    {
        $(
            $(#[cfg($($t:tt)*)])?
            $suite:ident
        ),*
    } => {
        $(
            $(#[cfg($($t)*)])?
            impl TryFrom<ssi_data_integrity_core::Type> for $suite {
                type Error = ssi_data_integrity_core::UnsupportedProofSuite;

                fn try_from(value: ssi_data_integrity_core::Type) -> Result<Self, Self::Error> {
                    let suite = $suite;

                    if value == <$suite as ssi_data_integrity_core::StandardCryptographicSuite>::type_(&suite) {
                        Ok($suite)
                    } else {
                        Err(ssi_data_integrity_core::UnsupportedProofSuite::Compact(value))
                    }
                }
            }
        )*
    };
}

try_from_type! {
    #[cfg(all(feature = "rsa", feature = "w3c"))]
    RsaSignature2018,

    #[cfg(all(feature = "ed25519", feature = "w3c"))]
    Ed25519Signature2018,

    #[cfg(all(feature = "ed25519", feature = "w3c"))]
    Ed25519Signature2020,

    #[cfg(all(feature = "ed25519", feature = "w3c"))]
    EdDsa2022,

    #[cfg(all(feature = "ed25519", feature = "w3c"))]
    EdDsaRdfc2022,

    #[cfg(all(feature = "secp256k1", feature = "w3c"))]
    EcdsaSecp256k1Signature2019,

    #[cfg(all(
        feature = "secp256r1",
        feature = "secp384r1",
        feature = "w3c"
    ))]
    EcdsaRdfc2019,

    #[cfg(all(feature = "secp256r1", feature = "w3c"))]
    EcdsaSd2023,

    #[cfg(all(feature = "eip712", feature = "w3c"))]
    EthereumEip712Signature2021,

    #[cfg(all(feature = "eip712", feature = "w3c"))]
    EthereumEip712Signature2021v0_1,

    #[cfg(all(feature = "secp256r1", feature = "w3c"))]
    EcdsaSecp256r1Signature2019,

    #[cfg(feature = "w3c")]
    JsonWebSignature2020,

    #[cfg(all(feature = "bbs", feature = "w3c"))]
    Bbs2023,

    #[cfg(all(feature = "secp256k1", feature = "dif"))]
    EcdsaSecp256k1RecoverySignature2020,

    #[cfg(feature = "aleo")]
    AleoSignature2021,

    #[cfg(all(feature = "ethereum", feature = "eip712"))]
    Eip712Signature2021,

    #[cfg(all(feature = "ethereum", feature = "secp256k1"))]
    EthereumPersonalSignature2021,

    #[cfg(all(feature = "ethereum", feature = "secp256k1"))]
    EthereumPersonalSignature2021v0_1,

    #[cfg(feature = "solana")]
    SolanaSignature2021,

    #[cfg(all(feature = "ed25519", feature = "tezos"))]
    Ed25519BLAKE2BDigestSize20Base58CheckEncodedSignature2021,

    #[cfg(all(feature = "secp256r1", feature = "tezos"))]
    P256BLAKE2BDigestSize20Base58CheckEncodedSignature2021,

    #[cfg(feature = "tezos")]
    TezosJcsSignature2021,

    #[cfg(feature = "tezos")]
    TezosSignature2021
}
