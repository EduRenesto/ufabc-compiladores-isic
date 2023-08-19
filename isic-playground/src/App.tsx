import { useState } from 'react';

import { init, compile_to_c, run_interpreter } from "isic-playground-glue";
import { EditorWindow } from './editor-window';
import { CodeResultWindow } from './code-results-window';
import { WindowsWrapper } from './App.styles';
import { DiagnosticsWindow } from './diagnostics-window';
import { InterpreterWindow } from './interpreter-window';

function App() {
  init();

  const [codeOutput, setCodeOutput] = useState<string | undefined>(undefined);

  const [interpreterInput, setInterpreterInput] = useState<string>("");
  const [interpreterOutput, setInterpreterOutput] = useState<string>("");

  const [errors, setErrors] = useState<string[]>([]);
  const [warns, setWarns] = useState<string[]>([]);

  const compile = (text: string) => {
    const result = compile_to_c(text);

    setCodeOutput(result.output_code);
    setErrors(result.errors);
    setWarns(result.warns);
  };

  const evaluate = (text: string) => {
    const result = run_interpreter(text, interpreterInput);

    setInterpreterOutput(result.output);
    setErrors(result.errors);
    setWarns(result.warns);
  }

  return (
    <>
      <h4>IsiLang Playground</h4>
      <WindowsWrapper>
        <EditorWindow onCompile={compile} onEvaluate={evaluate}/>
        <DiagnosticsWindow errors={errors} warns={warns}/>
        <CodeResultWindow code={codeOutput}/>
        <InterpreterWindow onInputChanged={setInterpreterInput} output={interpreterOutput}/>
      </WindowsWrapper>
    </>
  );
}

export default App;
