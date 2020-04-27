use hdk::error::ZomeApiResult;
use meta_traits::{GlobalEntryRef, InterDNADao};

use crate::InterDNA;

impl InterDNADao for InterDNA {
    fn create_link(source: GlobalEntryRef, target: GlobalEntryRef) -> ZomeApiResult<()> {
        Ok(())
    }

    fn remove_link(source: GlobalEntryRef, target: GlobalEntryRef) -> ZomeApiResult<()> {
        Ok(())
    }

    fn get_outgoing(source: GlobalEntryRef) -> ZomeApiResult<Vec<GlobalEntryRef>> {
        Ok(vec![])
    }

    fn get_incoming(target: GlobalEntryRef) -> ZomeApiResult<Vec<GlobalEntryRef>> {
        Ok(vec![])
    }
}
