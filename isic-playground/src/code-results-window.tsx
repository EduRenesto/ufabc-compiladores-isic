import React from "react";
import { Window } from "./window";

export type CodeResultProps = {
    code?: string,
};

export const CodeResultWindow: React.FC<CodeResultProps> = (props: CodeResultProps) => {
    return (
        <Window title="Code emitter output" disabled={!props.code}>
            <p>C code output</p>
            <pre>{props.code}</pre>
        </Window>
    )
};
