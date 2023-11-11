import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface ChangeRecord { 'change_type' : string, 'timestamp' : bigint }
export type Error = { 'NotFound' : { 'msg' : string } };
export interface ItemStatistics {
  'total_items' : bigint,
  'average_availability_rate' : number,
}
export type Query = { 'GetItem' : bigint };
export type QueryResult = { 'Error' : Error } |
  { 'Item' : SmartStorageItem };
export type Result = { 'Ok' : SmartStorageItem } |
  { 'Err' : Error };
export type Result_1 = { 'Ok' : boolean } |
  { 'Err' : Error };
export interface SmartStorageItem {
  'id' : bigint,
  'updated_at' : [] | [bigint],
  'name' : string,
  'description' : string,
  'created_at' : bigint,
  'is_available' : boolean,
  'location' : string,
}
export interface SmartStorageItemPayload {
  'name' : string,
  'description' : string,
  'is_available' : boolean,
  'location' : string,
}
export interface _SERVICE {
  'add_smart_storage_item' : ActorMethod<[SmartStorageItemPayload], Result>,
  'batch_query' : ActorMethod<[Array<Query>], Array<QueryResult>>,
  'delete_smart_storage_item' : ActorMethod<[bigint], Result>,
  'get_all_smart_storage_items' : ActorMethod<[], Array<SmartStorageItem>>,
  'get_available_smart_storage_items' : ActorMethod<
    [],
    Array<SmartStorageItem>
  >,
  'get_item_history' : ActorMethod<[bigint], Array<ChangeRecord>>,
  'get_item_statistics' : ActorMethod<[], ItemStatistics>,
  'get_smart_storage_item' : ActorMethod<[bigint], Result>,
  'is_item_available' : ActorMethod<[bigint], Result_1>,
  'mark_item_as_available' : ActorMethod<[bigint], Result>,
  'mark_item_as_unavailable' : ActorMethod<[bigint], Result>,
  'search_smart_storage_items' : ActorMethod<[string], Array<SmartStorageItem>>,
  'sort_items_by_name' : ActorMethod<[], Array<SmartStorageItem>>,
  'update_smart_storage_item' : ActorMethod<
    [bigint, SmartStorageItemPayload],
    Result
  >,
}
