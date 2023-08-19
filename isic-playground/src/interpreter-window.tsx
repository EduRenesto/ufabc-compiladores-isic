import React from "react";
import { Window } from "./window";

export type InterpreterProps = {
    onInputChanged: (input: string) => void,
    output: string,
};

export const InterpreterWindow: React.FC<InterpreterProps> = (props: InterpreterProps) => {
    return (
        <Window title="Interpreter">
            <p>Input</p>
            <textarea onChange={(e) => props.onInputChanged(e.target.value)}></textarea>
            <p>Output</p>
            <pre>{props.output}</pre>
        </Window>
    )
};
