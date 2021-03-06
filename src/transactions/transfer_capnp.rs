// Generated by the capnpc-rust plugin to the Cap'n Proto schema compiler.
// DO NOT EDIT.
// source: transactions/transfer.capnp


pub mod transfer {
  #[derive(Copy, Clone)]
  pub struct Owned;
  impl <'a> ::capnp::traits::Owned<'a> for Owned { type Reader = Reader<'a>; type Builder = Builder<'a>; }
  impl <'a> ::capnp::traits::OwnedStruct<'a> for Owned { type Reader = Reader<'a>; type Builder = Builder<'a>; }
  impl ::capnp::traits::Pipelined for Owned { type Pipeline = Pipeline; }

  #[derive(Clone, Copy)]
  pub struct Reader<'a> { reader: ::capnp::private::layout::StructReader<'a> }

  impl <'a,> ::capnp::traits::HasTypeId for Reader<'a,>  {
    #[inline]
    fn type_id() -> u64 { _private::TYPE_ID }
  }
  impl <'a,> ::capnp::traits::FromStructReader<'a> for Reader<'a,>  {
    fn new(reader: ::capnp::private::layout::StructReader<'a>) -> Reader<'a,> {
      Reader { reader: reader,  }
    }
  }

  impl <'a,> ::capnp::traits::FromPointerReader<'a> for Reader<'a,>  {
    fn get_from_pointer(reader: &::capnp::private::layout::PointerReader<'a>) -> ::capnp::Result<Reader<'a,>> {
      ::std::result::Result::Ok(::capnp::traits::FromStructReader::new(try!(reader.get_struct(::std::ptr::null()))))
    }
  }

  impl <'a,> ::capnp::traits::Imbue<'a> for Reader<'a,>  {
    fn imbue(&mut self, cap_table: &'a ::capnp::private::layout::CapTable) {
      self.reader.imbue(::capnp::private::layout::CapTableReader::Plain(cap_table))
    }
  }

  impl <'a,> Reader<'a,>  {
    pub fn borrow<'b>(&'b self) -> Reader<'b,> {
      Reader { .. *self }
    }

    pub fn total_size(&self) -> ::capnp::Result<::capnp::MessageSize> {
      self.reader.total_size()
    }
    #[inline]
    pub fn get_from(self) -> u32 {
      self.reader.get_data_field::<u32>(0)
    }
    #[inline]
    pub fn get_to(self) -> u32 {
      self.reader.get_data_field::<u32>(1)
    }
    #[inline]
    pub fn get_amount(self) -> u64 {
      self.reader.get_data_field::<u64>(1)
    }
    #[inline]
    pub fn get_seed(self) -> u64 {
      self.reader.get_data_field::<u64>(2)
    }
  }

  pub struct Builder<'a> { builder: ::capnp::private::layout::StructBuilder<'a> }
  impl <'a,> ::capnp::traits::HasStructSize for Builder<'a,>  {
    #[inline]
    fn struct_size() -> ::capnp::private::layout::StructSize { _private::STRUCT_SIZE }
  }
  impl <'a,> ::capnp::traits::HasTypeId for Builder<'a,>  {
    #[inline]
    fn type_id() -> u64 { _private::TYPE_ID }
  }
  impl <'a,> ::capnp::traits::FromStructBuilder<'a> for Builder<'a,>  {
    fn new(builder: ::capnp::private::layout::StructBuilder<'a>) -> Builder<'a, > {
      Builder { builder: builder,  }
    }
  }

  impl <'a,> ::capnp::traits::ImbueMut<'a> for Builder<'a,>  {
    fn imbue_mut(&mut self, cap_table: &'a mut ::capnp::private::layout::CapTable) {
      self.builder.imbue(::capnp::private::layout::CapTableBuilder::Plain(cap_table))
    }
  }

  impl <'a,> ::capnp::traits::FromPointerBuilder<'a> for Builder<'a,>  {
    fn init_pointer(builder: ::capnp::private::layout::PointerBuilder<'a>, _size: u32) -> Builder<'a,> {
      ::capnp::traits::FromStructBuilder::new(builder.init_struct(_private::STRUCT_SIZE))
    }
    fn get_from_pointer(builder: ::capnp::private::layout::PointerBuilder<'a>) -> ::capnp::Result<Builder<'a,>> {
      ::std::result::Result::Ok(::capnp::traits::FromStructBuilder::new(try!(builder.get_struct(_private::STRUCT_SIZE, ::std::ptr::null()))))
    }
  }

  impl <'a,> ::capnp::traits::SetPointerBuilder<Builder<'a,>> for Reader<'a,>  {
    fn set_pointer_builder<'b>(pointer: ::capnp::private::layout::PointerBuilder<'b>, value: Reader<'a,>) -> ::capnp::Result<()> { pointer.set_struct(&value.reader) }
  }

  impl <'a,> Builder<'a,>  {
    pub fn as_reader(self) -> Reader<'a,> {
      ::capnp::traits::FromStructReader::new(self.builder.as_reader())
    }
    pub fn borrow<'b>(&'b mut self) -> Builder<'b,> {
      Builder { .. *self }
    }
    pub fn borrow_as_reader<'b>(&'b self) -> Reader<'b,> {
      ::capnp::traits::FromStructReader::new(self.builder.as_reader())
    }

    pub fn total_size(&self) -> ::capnp::Result<::capnp::MessageSize> {
      self.builder.as_reader().total_size()
    }
    #[inline]
    pub fn get_from(self) -> u32 {
      self.builder.get_data_field::<u32>(0)
    }
    #[inline]
    pub fn set_from(&mut self, value: u32)  {
      self.builder.set_data_field::<u32>(0, value);
    }
    #[inline]
    pub fn get_to(self) -> u32 {
      self.builder.get_data_field::<u32>(1)
    }
    #[inline]
    pub fn set_to(&mut self, value: u32)  {
      self.builder.set_data_field::<u32>(1, value);
    }
    #[inline]
    pub fn get_amount(self) -> u64 {
      self.builder.get_data_field::<u64>(1)
    }
    #[inline]
    pub fn set_amount(&mut self, value: u64)  {
      self.builder.set_data_field::<u64>(1, value);
    }
    #[inline]
    pub fn get_seed(self) -> u64 {
      self.builder.get_data_field::<u64>(2)
    }
    #[inline]
    pub fn set_seed(&mut self, value: u64)  {
      self.builder.set_data_field::<u64>(2, value);
    }
  }

  pub struct Pipeline { _typeless: ::capnp::any_pointer::Pipeline }
  impl ::capnp::capability::FromTypelessPipeline for Pipeline {
    fn new(typeless: ::capnp::any_pointer::Pipeline) -> Pipeline {
      Pipeline { _typeless: typeless,  }
    }
  }
  impl Pipeline  {
  }
  mod _private {
    use capnp::private::layout;
    pub const STRUCT_SIZE: layout::StructSize = layout::StructSize { data: 3, pointers: 0 };
    pub const TYPE_ID: u64 = 0xc505ebd3d1f4ba85;
  }
}
