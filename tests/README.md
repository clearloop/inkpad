## Removed Tests

Tests below are removed from `pallet-contracts`, because `inkpad` doesn't care about these.


### Account

* ~~`calling_plain_account_fails`~~

* ~~`account_removal_does_not_remove_storage`~~

### Event

* ~~`deposit_event_max_value_limit`~~


### Gas

* ~~`run_out_of_gas`~~

* ~~`reinstrument_does_charge`~~


### Storage

* ~~`storage_size`~~

* ~~`empty_kv_pairs`~~

* ~~`storage_max_value_limit`~~


### Fees

* ~~`deduct_blocks`~~

* ~~`default_rent_allowance_on_instantiate`~~

* ~~`claim_surcharge_malus`~~

* ~~`surcharge_reward_is_capped`~~

* ~~`refcounter`~~


### Removals

* ~~`inherent_claim_surcharge_contract_removals`~~

* ~~`signed_claim_surcharge_contract_removals`~~

* ~~`lazy_removal_works`~~

* ~~`lazy_removal_partial_remove_works`~~

* ~~`lazy_removal_does_no_run_on_full_block`~~

* ~~`lazy_removal_does_not_use_all_weight`~~

* ~~`deletion_queue_full`~~

* ~~`call_removed_contract`~~

## Restoration

* ~~`restorations_dirty_storage_and_different_storage`~~

* ~~`restorations_dirty_storage`~~

* ~~`restoration_different_storage`~~

* ~~`restoration_code_evicted`~~

* ~~`restoration_success`~~

## Drain

* ~~`cannot_self_destruct_through_draning`~~

* ~~`cannot_self_destruct_while_live`~~

* ~~`self_destruct_works`~~

* ~~`destroy_contract_and_transfer_funds`~~

* ~~`cannot_self_destruct_in_constructor`~~

## Chain Extension

* ~~`disabled_chain_extension_wont_deploy`~~

* ~~`disabled_chain_extension_errors_on_call`~~

* ~~`chain_extension_works`~~
