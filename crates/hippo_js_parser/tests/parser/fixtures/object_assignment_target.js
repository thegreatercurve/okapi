({} = {});
({ bar, baz } = {});
({
  bar: [baz = "baz"],
  foo = "foo",
  ...ret
} = {});
