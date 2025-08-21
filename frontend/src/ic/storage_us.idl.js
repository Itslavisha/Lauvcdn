export default ({ IDL }) => {
  const Chunk = IDL.Record({ data: IDL.Vec(IDL.Nat8), sha256: IDL.Vec(IDL.Nat8) })
  const FileMeta = IDL.Record({
    file_id: IDL.Text,
    total_size: IDL.Nat64,
    chunk_size: IDL.Nat32,
    num_chunks: IDL.Nat32,
    merkle_root: IDL.Vec(IDL.Nat8),
    created_ns: IDL.Nat64,
  })
  return IDL.Service({
    health: IDL.Func([], [IDL.Text], ['query']),
    put_chunk: IDL.Func([IDL.Text, IDL.Nat32, IDL.Vec(IDL.Nat8), IDL.Vec(IDL.Nat8)], [], []),
    finalize_file: IDL.Func([FileMeta], [], []),
    get_chunk: IDL.Func([IDL.Text, IDL.Nat32], [IDL.Opt(Chunk)], ['query']),
    get_file_meta: IDL.Func([IDL.Text], [IDL.Opt(FileMeta)], ['query']),
  })
}
