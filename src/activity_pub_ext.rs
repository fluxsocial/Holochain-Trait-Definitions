use hdk::holochain_json_api::json::JsonString;
use hdk::{
    error::{ZomeApiResult, ZomeApiError},
    holochain_core_types::{chain_header::ChainHeader, entry::Entry},
    holochain_persistence_api::cas::content::Address,
};

use crate::GlobalEntryRef;

pub enum Method {
    Post,
    Get,
    Put,
    Delete
}

/// Describes a DNA method 
pub struct DnaMethod {
    pub dna: Address,
    pub resource: Option<String>, //eg get_by_address
    pub method: Method,
    pub params: Option<JsonString> // Params for function
}

//Used to represent methods for a given "resource"
//needed since holochain does not support multiple HTTP methods to one endpoint; just supports POST to every endpoint
//alternative for this is having some string encoded representation such as
//dna-address;resource-get;method;params\dna-address;resource-post;method;params
pub struct MethodPair {
    pub post: Option<DnaMethod>,
    pub get: Option<DnaMethod>
}

pub struct ApActor {
    pub context: String, //Likely need to define our own context that extends from Activity Streams to incorporate pub/private resources
    pub actor: Box<dyn activitystreams::actor::AsApActor<dyn activitystreams::markers::Actor>>,
    //Likely that all DNA methods below would point to social contexts
    //References to public DNA's
    //Since auth is not possible on DNA's and instead they are protected by membrane rules; we need different DNA's for different privacy levels
    pub inbox_pub: Option<MethodPair>, //Likely a social context w/ resources for post'ing there and getting actors post there
    pub outbox_pub: Option<MethodPair>, //Likely a social context w/ resources for post'ing there and getting actors post there
    pub followers_pub: Option<MethodPair>, //Likely a social graph w/ methods for getting followers and creating new follow
    pub following_pub: Option<MethodPair>, //Likely a social graph w/ methods for getting followers and creating new follow
    pub collections_public: Box<dyn activitystreams::markers::Collection>, //Various collections of expressions/social contexts
    //References to private DNA's
    pub inbox_private: Option<MethodPair>, //Likely a social context w/ resources for post'ing there and getting actors post there
    pub outbox_private: Option<MethodPair>, //Likely a social context w/ resources for post'ing there and getting actors post there
    pub followers_private: Option<MethodPair>, //Likely a social graph w/ methods for getting followers and creating new follow
    pub following_private: Option<MethodPair>, //Likely a social graph w/ methods for getting followers and creating new follow
    pub collections_private: Box<dyn activitystreams::markers::Collection> //Various collections of expressions/social contexts
}

pub trait APProfile {
    fn create_profile(actor_data: ApActor) -> ZomeApiResult<ApActor>;
    fn get_profile(target: Address) -> ZomeApiResult<Option<ApActor>>;
    fn update_profile(actor_data: ApActor) -> ZomeApiResult<ApActor>;
    fn delete_profile() -> ZomeApiResult<()>;
}

pub trait SocialGraph {
    // Follow Related Operations
    // Inner values for collections here likely Object of type relationship
    fn my_followers(relationship: Option<String>) -> activitystreams::collection::OrderedCollection;
    fn followers(followed_agent: Address, relationship: Option<String>) -> activitystreams::collection::OrderedCollection;
    fn nth_level_followers(n: u32, followed_agent: Address, relationship: Option<String>) -> activitystreams::collection::OrderedCollection;

    fn my_followings(relationship: Option<String>) -> activitystreams::collection::OrderedCollection;
    fn following(following_agent: Address, relationship: Option<String>) -> activitystreams::collection::OrderedCollection;
    fn nth_level_following(n: u32, following_agent: Address, relationship: Option<String>) -> activitystreams::collection::OrderedCollection;

    fn follow(other_agent: Address, relationship: Option<String>) -> Result<(), ZomeApiError>;
    fn unfollow(other_agent: Address, relationship: Option<String>) -> Result<(), ZomeApiError>;

    // Connection Related Operations (i.e. bidirectional friendship)
    fn my_friends() -> activitystreams::collection::OrderedCollection;
    fn friends_of(agent: Address) -> activitystreams::collection::OrderedCollection;

    fn request_friendship(other_agent: Address);
    fn decline_friendship(other_agent: Address);

    fn incoming_friendship_requests() -> activitystreams::collection::OrderedCollection;
    fn outgoing_friendship_requests() -> activitystreams::collection::OrderedCollection;

    fn drop_friendship(other_agent: Address) -> Result<(), ZomeApiError>;
}

/// A holochain expression
pub struct HolochainExpression {
    pub entry: Entry,
    pub headers: Vec<ChainHeader>,
    pub expression_dna: Address,
    pub activity_streams_entry: Box<dyn activitystreams::markers::Object>,
    pub inter_dna_link_dna: Option<Address>,
}

/// An interface into a DNA which contains Expression information. Expected to be interacted with using expression Addresses
/// retrieved from a social context or by using a Identity retreived from a users social graph.
/// In this situation you can see that the Expression DNA/trait does not need to include any index capability
/// as this is already infered to the agent by the place they got the expression from; social context or social graph.
///
/// If the expression should be private to a group of people then the host DNA should be membraned.
pub trait Expression {
    /// Create an expression and link it to yourself publicly with optional dna_address pointing to
    /// dna that should ideally be used for linking any comments to this expression
    fn create_public_expression(content: String, inter_dna_link_dna: Option<Address>) -> HolochainExpression;
    /// Get expressions authored by a given Agent/Identity
    fn get_by_author(author: Address, count: u32, page: u32) -> Vec<HolochainExpression>;
    fn get_expression_by_address(address: Address) -> Option<HolochainExpression>;

    /// Send an expression to someone privately p2p
    fn send_private(to: Address, content: String, inter_dna_link_dna: Option<Address>);
    /// Get private expressions sent to you
    fn inbox() -> Vec<HolochainExpression>;
}

/// Interface for cross DNA links. Allows for the discovery of new DNA's/entries from a known source DNA/entry.
/// Host DNA should most likely implement strong anti spam logic if this is to be a public - unmembraned DNA.
pub trait InterDNA {
    fn create_link(source: GlobalEntryRef, target: GlobalEntryRef) -> Box<dyn activitystreams::markers::Object>;
    fn remove_link(source: GlobalEntryRef, target: GlobalEntryRef) -> Box<dyn activitystreams::markers::Object>;

    fn get_outgoing(source: GlobalEntryRef, filter_dna: Address) -> activitystreams::collection::OrderedCollection;
    fn get_incoming(target: GlobalEntryRef, filter_dna: Address) -> activitystreams::collection::OrderedCollection;
}


//Note holochain zome handlers will need to parse incoming json and serialize into types that implement required traits 
//for traits above