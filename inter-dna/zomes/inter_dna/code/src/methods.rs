use hdk::{
    error::{ZomeApiError, ZomeApiResult},
    holochain_json_api::json::JsonString,
    prelude::{Address, Entry, GetLinksOptions, LinkMatch, Pagination, SizePagination, SortOrder},
};
use meta_traits::{GlobalEntryRef, InterDNADao};
use std::convert::TryInto;

use crate::InterDNA;

impl InterDNADao for InterDNA {
    fn create_link(source: GlobalEntryRef, target: GlobalEntryRef) -> ZomeApiResult<()> {
        let source_address: Address = JsonString::from(source.clone()).try_into()?;
        if hdk::get_entry(&source_address)?.is_none() {
            let source_entry = Entry::App("global_entry_ref".into(), source.into());
            hdk::commit_entry(&source_entry)?;
        };

        let target_address: Address = JsonString::from(target.clone()).try_into()?;
        if hdk::get_entry(&target_address)?.is_none() {
            let target_entry = Entry::App("global_entry_ref".into(), target.into());
            hdk::commit_entry(&target_entry)?;
        };

        hdk::link_entries(&source_address, &target_address, "", "")?;
        hdk::link_entries(&target_address, &source_address, "incoming", "")?;
        Ok(())
    }

    fn remove_link(source: GlobalEntryRef, target: GlobalEntryRef) -> ZomeApiResult<()> {
        let source_address: Address = JsonString::from(source.clone()).try_into()?;
        hdk::get_entry(&source_address)?.ok_or(ZomeApiError::Internal(String::from(
            "Source entry does not exist",
        )))?;

        let target_address: Address = JsonString::from(target.clone()).try_into()?;
        hdk::get_entry(&target_address)?.ok_or(ZomeApiError::Internal(String::from(
            "Target entry does not exist",
        )))?;

        hdk::remove_link(&source_address, &target_address, "", "")?;
        Ok(())
    }

    fn get_outgoing(
        source: GlobalEntryRef,
        count: usize,
        page: usize,
    ) -> ZomeApiResult<Vec<GlobalEntryRef>> {
        let source_address: Address = JsonString::from(source.clone()).try_into()?;
        Ok(hdk::get_links_with_options(
            &source_address,
            LinkMatch::Any,
            LinkMatch::Any,
            GetLinksOptions {
                status_request: Default::default(),
                headers: false,
                timeout: Default::default(),
                pagination: Some(Pagination::Size(SizePagination {
                    page_number: page,
                    page_size: count,
                })),
                sort_order: Some(SortOrder::Descending),
            },
        )?
        .addresses()
        .into_iter()
        .map(|link_target_address| hdk::utils::get_as_type::<GlobalEntryRef>(link_target_address))
        .collect::<ZomeApiResult<Vec<GlobalEntryRef>>>()?)
    }

    fn get_incoming(
        target: GlobalEntryRef,
        count: usize,
        page: usize,
    ) -> ZomeApiResult<Vec<GlobalEntryRef>> {
        let target_address: Address = JsonString::from(target.clone()).try_into()?;
        Ok(hdk::get_links_with_options(
            &target_address,
            LinkMatch::Exactly("incoming"),
            LinkMatch::Any,
            GetLinksOptions {
                status_request: Default::default(),
                headers: false,
                timeout: Default::default(),
                pagination: Some(Pagination::Size(SizePagination {
                    page_number: page,
                    page_size: count,
                })),
                sort_order: Some(SortOrder::Descending),
            },
        )?
        .addresses()
        .into_iter()
        .map(|link_target_address| hdk::utils::get_as_type::<GlobalEntryRef>(link_target_address))
        .collect::<ZomeApiResult<Vec<GlobalEntryRef>>>()?)
    }
}
