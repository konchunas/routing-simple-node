@0xfa882690d04fe2c4;

struct Transfer {
    from @0        :UInt32; #public keys
    to @1          :UInt32; #public keys
    amount @2      :UInt64;
    seed @3        :UInt64;
}