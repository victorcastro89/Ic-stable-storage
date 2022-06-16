import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface Chunk {
  'chunk_index' : bigint,
  'chunk' : Array<number>,
  'filename' : string,
}
export interface _SERVICE {
  'commit_batch' : ActorMethod<[string, Array<string>, string], string>,
  'create_chunk' : ActorMethod<[Chunk], string>,
  'read' : ActorMethod<[bigint, bigint], Array<number>>,
  'stablegrow' : ActorMethod<[bigint], bigint>,
  'stablesize' : ActorMethod<[], bigint>,
}
