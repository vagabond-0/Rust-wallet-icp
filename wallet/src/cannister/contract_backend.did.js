export const idlFactory = ({ IDL }) => {
  const User = IDL.Record({ 'username' : IDL.Text, 'balance' : IDL.Nat64 });
  const Result = IDL.Variant({ 'Ok' : IDL.Null, 'Err' : IDL.Text });
  return IDL.Service({
    'create_account' : IDL.Func([User], [IDL.Opt(User)], []),
    'get_balance' : IDL.Func([], [IDL.Nat64], ['query']),
    'get_self' : IDL.Func([], [User], ['query']),
    'transfer_tokens' : IDL.Func([IDL.Principal, IDL.Nat64], [Result], []),
  });
};
export const init = ({ IDL }) => { return []; };
