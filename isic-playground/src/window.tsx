import React, { ReactElement } from "react";
import { WindowWrapper } from "./window.style";

export type WindowProps = {
    title: string,
    disabled?: boolean,
    children?: ReactElement[],
};

export const Window: React.FC<WindowProps> = (props: WindowProps) => {
    const titleClassName = props.disabled ? "title-bar inactive" : "title-bar";

    return (
        <WindowWrapper>
            <div className="window">
              <div className={titleClassName}>
                  <div className="title-bar-text">{props.title}</div>
                    <div className="title-bar-controls">
                        <button aria-label="Close"></button>
                    </div>
                </div>
                <div className="window-body">
                  { props.children }
                </div>
            </div>
        </WindowWrapper>
    )
}
