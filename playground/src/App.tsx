import { CodeEditor } from "./CodeEditor";

export default function App() {
  return (
    <div className="grid grid-cols-2 gap-4">
      <div>
        <CodeEditor />
      </div>
      <div>
        <CodeEditor />
      </div>
    </div>
  );
}
