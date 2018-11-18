import * as React from "react";
import { AnyNote } from "../../types";

export interface NoteEditorProps {
  onChange: (note: AnyNote) => {};
  onDone: () => {};
  note: AnyNote;
}

export default class NoteEditor extends React.PureComponent<
  NoteEditorProps,
  any
> {
  handleChange = (e: any) => {
    const content = e.target.value;

    this.props.onChange({ ...this.props.note, content });
  };

  handleDone = () => {
    this.props.onDone();
  };

  render() {
    return (
      <div className="note-editor">
        <textarea
          onChange={this.handleChange}
          value={this.props.note.content}
        />

        <footer className="note-editor__footer">
          <button onClick={this.handleDone}>Done</button>
        </footer>
      </div>
    );
  }
}
