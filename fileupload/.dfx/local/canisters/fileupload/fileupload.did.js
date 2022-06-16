export const idlFactory = ({ IDL }) => {
  const Chunk = IDL.Record({
    'chunk_index' : IDL.Nat64,
    'chunk' : IDL.Vec(IDL.Nat8),
    'filename' : IDL.Text,
  });
  return IDL.Service({
    'commit_batch' : IDL.Func(
        [IDL.Text, IDL.Vec(IDL.Text), IDL.Text],
        [IDL.Text],
        [],
      ),
    'create_chunk' : IDL.Func([Chunk], [IDL.Text], []),
    'read' : IDL.Func([IDL.Nat64, IDL.Nat64], [IDL.Vec(IDL.Nat8)], ['query']),
    'stablegrow' : IDL.Func([IDL.Nat64], [IDL.Nat64], []),
    'stablesize' : IDL.Func([], [IDL.Nat64], ['query']),
  });
};
export const init = ({ IDL }) => { return []; };
