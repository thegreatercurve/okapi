import Editor from "@monaco-editor/react";

const DEFAULT_CODE = `function hello() {
  return 'world';
}`;

export const CodeEditor = () => {
  return (
    <Editor
      height="300px"
      defaultLanguage="javascript"
      defaultValue={DEFAULT_CODE}
    />
  );
};
