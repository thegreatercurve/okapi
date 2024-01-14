let a = {
  set foo(value) {},
  set bar(value) {},
  set ["a" + "b"](value) {},
  set 5(value) {},
  set() {
    return "Thi i a method and not a setter";
  },
};
