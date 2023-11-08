import { useRef, useEffect, useState } from "react";

import { EditorView } from "@codemirror/view";
import { javascript } from "@codemirror/lang-javascript";
import { EditorState } from "@codemirror/state";

export const Editor = () => {
  const editor = useRef<HTMLDivElement | null>(null);
  const [code, setCode] = useState("");

  const onUpdate = EditorView.updateListener.of((v) => {
    setCode(v.state.doc.toString());

    console.log(code);
  });

  useEffect(() => {
    const state = EditorState.create({
      doc: "Hello World",
      extensions: [javascript()],
    });

    if (editor.current === null) {
      return;
    }

    const view = new EditorView({ state, parent: editor.current });

    return () => {
      view.destroy();
    };
  }, [onUpdate]);

  return <div ref={editor}></div>;
};
