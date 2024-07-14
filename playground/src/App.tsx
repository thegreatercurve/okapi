import { Editor } from "@monaco-editor/react";
import { useEffect, useState } from "react";
import init, { parseModule } from "okapi_wasm/okapi_wasm";

const options = {
  readOnly: false,
  minimap: { enabled: false },
};

const App = () => {
  const [isWASMLoaded, setIsWASMLoaded] = useState(false);
  const [codeValue, setCodeValue] = useState<
    string | null
  >(`function helloWorld() {
  return; 
}`);
  const [previewValue, setPreviewValue] = useState<
    string | null
  >(`function helloWorld() {
  return; 
}`);

  useEffect(() => {
    init().then(() => {
      setIsWASMLoaded(true);
    });
  }, []);

  useEffect(() => {
    if (isWASMLoaded && codeValue !== null) {
      try {
        setPreviewValue(parseModule(codeValue));
      } catch (e) {
        console.error(e);
      }
    }
  }, [codeValue, isWASMLoaded]);

  const onChange = (value: string | undefined) => {
    setCodeValue(value ?? null);
  };

  return (
    <>
      <Editor
        height="50vh"
        defaultLanguage="javascript"
        options={options}
        onChange={onChange}
        defaultValue="function helloWorld() {
  return; 
}"
      />
      <div>
        <h2>Output</h2>
        {isWASMLoaded && previewValue !== null ? (
          <pre className="code-preview">{previewValue}</pre>
        ) : (
          <div>Loading...</div>
        )}
      </div>
    </>
  );
};

export default App;
