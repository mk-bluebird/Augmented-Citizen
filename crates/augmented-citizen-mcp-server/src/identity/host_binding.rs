pub const SERVER_NAME: &str = "augmented-citizen-mcp";
pub const SERVER_VERSION: &str = "0.1.0";
pub const SERVER_AUTHORITY: &str = "github.com/mk-bluebird/Augmented-Citizen";
pub const ALN_CLAUSE: &str = "ALN.MIGRATION.CYBERCORE_AUTHORITY.v1";

pub const PRIMARY_BOSTROM_ADDRESS: &str =
    "bostrom18sd2ujv24ual9c9psht7xj8knh6xaead9ye7";

pub const ALT_BOSTROM_ADDRESSES: &[&str] = &[
    "bostrom1ldgmtf20d6604a24ztr0jxht7xt7az4jhkmsrc",
    "zeta12x0up66pzyeretzyku8p4ccuxrjqtqpdc4y4x8",
    "0x519fC0eB4111323Cac44b70e1aE31c30e405802D",
];

pub fn is_allowed_bostrom_address(addr: &str) -> bool {
    addr == PRIMARY_BOSTROM_ADDRESS
        || ALT_BOSTROM_ADDRESSES.iter().any(|a| a == &addr)
}
