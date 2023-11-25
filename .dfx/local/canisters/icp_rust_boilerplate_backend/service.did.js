export const idlFactory = ({ IDL }) => {
  const SmartStorageItemPayload = IDL.Record({
    'name' : IDL.Text,
    'description' : IDL.Text,
    'is_available' : IDL.Bool,
    'location' : IDL.Text,
  });
  const SmartStorageItem = IDL.Record({
    'id' : IDL.Nat64,
    'updated_at' : IDL.Opt(IDL.Nat64),
    'name' : IDL.Text,
    'description' : IDL.Text,
    'created_at' : IDL.Nat64,
    'is_available' : IDL.Bool,
    'location' : IDL.Text,
  });
  const Query = IDL.Variant({ 'GetItem' : IDL.Nat64 });
  const Error = IDL.Variant({ 'NotFound' : IDL.Record({ 'msg' : IDL.Text }) });
  const QueryResult = IDL.Variant({
    'Error' : Error,
    'Item' : SmartStorageItem,
  });
  const Result = IDL.Variant({ 'Ok' : SmartStorageItem, 'Err' : Error });
  const ChangeRecord = IDL.Record({
    'change_type' : IDL.Text,
    'timestamp' : IDL.Nat64,
  });
  const ItemStatistics = IDL.Record({
    'total_items' : IDL.Nat64,
    'average_availability_rate' : IDL.Float64,
  });
  const TransactionRecord = IDL.Record({
    'transaction_type' : IDL.Text,
    'change_type' : IDL.Text,
    'timestamp' : IDL.Nat64,
  });
  const Result_1 = IDL.Variant({ 'Ok' : IDL.Bool, 'Err' : Error });
  return IDL.Service({
    'add_smart_storage_item' : IDL.Func(
        [SmartStorageItemPayload],
        [IDL.Opt(SmartStorageItem)],
        [],
      ),
    'batch_query' : IDL.Func(
        [IDL.Vec(Query)],
        [IDL.Vec(QueryResult)],
        ['query'],
      ),
    'bulk_update_smart_storage_items' : IDL.Func(
        [IDL.Vec(IDL.Tuple(IDL.Nat64, SmartStorageItemPayload))],
        [IDL.Vec(Result)],
        [],
      ),
    'delete_smart_storage_item' : IDL.Func([IDL.Nat64], [Result], []),
    'get_all_smart_storage_items' : IDL.Func(
        [],
        [IDL.Vec(SmartStorageItem)],
        ['query'],
      ),
    'get_available_smart_storage_items' : IDL.Func(
        [],
        [IDL.Vec(SmartStorageItem)],
        ['query'],
      ),
    'get_item_history' : IDL.Func(
        [IDL.Nat64],
        [IDL.Vec(ChangeRecord)],
        ['query'],
      ),
    'get_item_statistics' : IDL.Func([], [ItemStatistics], ['query']),
    'get_item_transaction_history' : IDL.Func(
        [IDL.Nat64],
        [IDL.Vec(TransactionRecord)],
        ['query'],
      ),
    'get_paginated_smart_storage_items' : IDL.Func(
        [IDL.Nat64, IDL.Nat64],
        [IDL.Vec(SmartStorageItem)],
        ['query'],
      ),
    'get_smart_storage_item' : IDL.Func([IDL.Nat64], [Result], ['query']),
    'is_item_available' : IDL.Func([IDL.Nat64], [Result_1], ['query']),
    'mark_item_as_available' : IDL.Func([IDL.Nat64], [Result], []),
    'mark_item_as_unavailable' : IDL.Func([IDL.Nat64], [Result], []),
    'search_smart_storage_items' : IDL.Func(
        [IDL.Text],
        [IDL.Vec(SmartStorageItem)],
        ['query'],
      ),
    'sort_items_by_name' : IDL.Func([], [IDL.Vec(SmartStorageItem)], ['query']),
    'update_smart_storage_item' : IDL.Func(
        [IDL.Nat64, SmartStorageItemPayload],
        [Result],
        [],
      ),
  });
};
export const init = ({ IDL }) => { return []; };
