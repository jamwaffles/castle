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
  handleChangeContent = (e: any) => {
    const content = e.target.value;

    this.props.onChange({ ...this.props.note, content });
  }

  handleChangeTitle = (e: any) => {
    const title = e.target.value;

    this.props.onChange({ ...this.props.note, title });
  }

  handleDone = () => {
    this.props.onDone();
  };

  render() {
    return (
      <div className="note-editor">
        <input type="text" onChange={this.handleChangeTitle} value={this.props.note.title} />

        <textarea
          onChange={this.handleChangeContent}
          value={this.props.note.content}
        />

        <footer className="note-editor__footer">
          <button onClick={this.handleDone}>Done</button>
        </footer>
      </div>
    );
  }
}
