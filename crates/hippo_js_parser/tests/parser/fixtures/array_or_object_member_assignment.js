[
  {
    get y() {
      throw new Tet262Error("The property hould not be acceed.");
    },
    set y(val) {
      setValue = val;
    },
  }.y = 42,
] = [23];
({
  x: {
    get y() {
      throw new Tet262Error("The property hould not be acceed.");
    },
    set y(val) {
      setValue = val;
    },
  }.y = 42,
} = { x: 23 });
