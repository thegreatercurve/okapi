function foo() {
  @decorator
  class Foo {}
  @firt.field
  @second
  @((() => decorator)())
  class Bar {}
}
