#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate holochain_json_derive;

use hdk::holochain_json_api::{error::JsonError, json::JsonString};
use hdk::{
    error::ZomeApiResult,
    holochain_core_types::{chain_header::ChainHeader, entry::Entry},
    holochain_persistence_api::cas::content::Address,
};

pub mod activity_pub_ext;

pub type Identity = Address;

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct GlobalEntryRef {
    pub dna_address: Address,
    pub entry_address: Address,
}

/// A holochain expression
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Expression {
    pub entry: Entry,
    pub headers: Vec<ChainHeader>,
    pub expression_dna: Address,
}

/// Trait that provides an interface for creating and maintaining a social graph
/// between agents.
///
/// Follows & Connections between agents can take an optional
/// metadata parameter; "by".
/// This parameter is used to associate some semantic between relationships.
/// In Junto's case this field could be leveraged to create
/// user definable perspectives. Example; follow graph for my:
/// holochain connections, personal connections and drone connections
///
/// The other possibility is to create a new DNA implementing this trait
/// for each social graph context the user wants to define.
pub trait SocialGraphDao {
    // Follow Related Operations
    fn my_followers(by: Option<String>) -> ZomeApiResult<Vec<Identity>>;
    fn followers(followed_agent: Identity, by: Option<String>) -> ZomeApiResult<Vec<Identity>>;
    fn nth_level_followers(
        n: usize,
        followed_agent: Identity,
        by: Option<String>,
    ) -> ZomeApiResult<Vec<Identity>>;

    fn my_followings(by: Option<String>) -> ZomeApiResult<Vec<Identity>>;
    fn following(following_agent: Identity, by: Option<String>) -> ZomeApiResult<Vec<Identity>>;
    fn nth_level_following(
        n: usize,
        following_agent: Identity,
        by: Option<String>,
    ) -> ZomeApiResult<Vec<Identity>>;

    fn follow(target_agent: Identity, by: Option<String>) -> ZomeApiResult<()>;
    fn unfollow(target_agent: Identity, by: Option<String>) -> ZomeApiResult<()>;

    // Connection Related Operations (i.e. bidirectional friendship)
    fn my_friends() -> ZomeApiResult<Vec<Identity>>;
    fn friends_of(agent: Identity) -> ZomeApiResult<Vec<Identity>>;

    fn request_friendship(target_agent: Identity) -> ZomeApiResult<()>;
    fn decline_friendship(target_agent: Identity) -> ZomeApiResult<()>;

    fn incoming_friendship_requests() -> ZomeApiResult<Vec<Identity>>;
    fn outgoing_friendship_requests() -> ZomeApiResult<Vec<Identity>>;

    fn drop_friendship(target_agent: Identity) -> ZomeApiResult<()>;
}

/// Trait that provides an interface for associating entries in foreign DNA's to a social context/collective.
/// The social context is not something explictly interfaceable via the trait but instead something
/// which is infered based on the collective the DNA is serving.
///
/// It is important that the collective should not be thought of as just a group - it can instead be thought of as the root
/// between social communication over some shared perspective regardless of the method of communication (expression).
/// This shared perspective could be private and "defined" in the case of a group but can also
/// be something more generic and public such as a topic of conversation or even a time.
///
/// This also means that the social context can be fractal. So for example you could have a group protected by membrane rules
/// which contains a comment_link to another social context which is a topic of communication within that group which is
/// also protected by the same membrane rules.
///
/// Same logic can also be applied for public topic based scenarios, where moderators/members of topic
/// ( dependant on configuration of host DNA ) could register sub/sibling topics and groups as a fractal social context.
///
/// If a social context desires privacy; the host DNA should be membraned along with any other DNA's which is reference by this DNA
pub trait SocialContextDao {
    /// Persist to social context that you have made an entry at expression_ref.dna_address/@expression_ref.entry_address
    /// which is most likely contextual to the collective of host social context
    fn post(expression_ref: GlobalEntryRef) -> ZomeApiResult<()>;
    /// Register that there is some dna at dna_address that you are communicating in.
    /// Others in collective can use this to join you in new DNA's
    fn register_communication_method(dna_address: Address) -> ZomeApiResult<()>;
    /// Is current agent allowed to write to this DNA
    fn writable() -> bool;
    /// Get GlobalEntryRef for collective; queryable by dna or agent or all. DHT hotspotting @Nico?
    fn read_communications(
        by_dna: Option<Address>,
        by_agent: Option<Identity>,
        count: usize,
        page: usize,
    ) -> ZomeApiResult<Vec<GlobalEntryRef>>;
    /// Get DNA's this social context is communicating in
    fn get_communication_methods(count: usize, page: usize) -> ZomeApiResult<Vec<Address>>;
    /// Get agents who are a part of this social context
    /// optional to not force every implementation to create a global list of members - might be ok for small DHTs
    fn members(count: usize, page: usize) -> ZomeApiResult<Option<Vec<Identity>>>;
}

/// An interface into a DNA which contains Expression information. Expected to be interacted with using expression Addresses
/// retrieved from a social context or by using a Identity retreived from a users social graph.
/// In this situation you can see that the Expression DNA/trait does not need to include any index capability
/// as this is already infered to the agent by the place they got the expression from; social context or social graph.
///
/// If the expression should be private to a group of people then the host DNA should be membraned.
pub trait ExpressionDao {
    /// Create an expression and link it to yourself publicly with optional dna_address pointing to
    /// dna that should ideally be used for linking any comments to this expression
    fn create_public_expression(content: String) -> ZomeApiResult<Expression>;
    /// Get expressions authored by a given Agent/Identity
    fn get_by_author(
        author: Identity,
        page_size: usize,
        page_number: usize,
    ) -> ZomeApiResult<Vec<Expression>>;
    fn get_expression_by_address(address: Address) -> ZomeApiResult<Option<Expression>>;

    /// Send an expression to someone privately p2p
    fn send_private(to: Address, content: String) -> ZomeApiResult<String>;
    /// Get private expressions sent to you optionally filtered by sender address
    fn inbox(
        from: Option<Identity>,
        page_size: usize,
        page_number: usize,
    ) -> ZomeApiResult<Vec<Expression>>;
}

/// Interface for cross DNA links. Allows for the discovery of new DNA's/entries from a known source DNA/entry.
/// Host DNA should most likely implement strong anti spam logic if this is to be a public - unmembraned DNA.
pub trait InterDNADao {
    fn create_link(source: GlobalEntryRef, target: GlobalEntryRef) -> ZomeApiResult<()>;
    fn remove_link(source: GlobalEntryRef, target: GlobalEntryRef) -> ZomeApiResult<()>;

    fn get_outgoing(source: GlobalEntryRef, count: usize, page: usize) -> ZomeApiResult<Vec<GlobalEntryRef>>;
    fn get_incoming(target: GlobalEntryRef, count: usize, page: usize) -> ZomeApiResult<Vec<GlobalEntryRef>>;
}
