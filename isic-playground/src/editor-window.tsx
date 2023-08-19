import React, { useState } from "react";

import "ace-builds/src-noconflict/ace";
import "ace-builds/src-noconflict/mode-java";
import "ace-builds/src-noconflict/theme-terminal";

import AceEditor from "react-ace";
import { ButtonsWrapper, WindowWrapper } from "./editor-window.style";
import { Window } from "./window";

const sampleText = `programa
    declare a: int.
    declare b: int.

    a := 10.
    leia(b).

    declare c: int.
    c := a * b.

    declare d: float.

    escreva(c).
fimprog.`;

export type EditorProps = {
    onCompile: (text: string) => void;
    onEvaluate: (text: string) => void;
};

export const EditorWindow: React.FC<EditorProps> = (props: EditorProps) => {
    const [text, setText] = useState(sampleText);

    return (
        <Window title="playground.isi - Editor">
            <AceEditor
                onChange={setText}
                value={text}
            />

            <ButtonsWrapper>
                <button onClick={() => props.onCompile(text)}>Compile</button>
                <button onClick={() => props.onEvaluate(text)}>Run interpreter</button>
            </ButtonsWrapper>
        </Window>
    )
}
