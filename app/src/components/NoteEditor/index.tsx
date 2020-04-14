import * as React from "react";
import { AnyNote } from "../../types";
import { Option } from "funfix";

import EditChecklistNote from './EditChecklistNote';
import EditTextNote from './EditTextNote';

export interface NoteEditorProps {
  onChange: (note: AnyNote) => {};
  onDone: () => {};
  note: Option<AnyNote>;
}

export default class NoteEditor extends React.PureComponent<
  NoteEditorProps,
  any
> {
  handleChangeContent = (e: any) => {
    const content = e.target.value;

    this.props.onChange({ ...this.props.note, content });
  };

  handleChangeTitle = (e: any) => {
    const title = e.target.value;

    this.props.onChange({ ...this.props.note, title });
  };

  handleDone = () => {
    this.props.onDone();
  };

  render() {
    return (
      <div className="note-editor">
        <input type="text" onChange={this.handleChangeTitle} value={title} />

        {this.props.note
          .map(note => {
            switch (note.note_type) {
              case "Checklist":
                return <EditChecklistNote note={note} />;

              case "Text":
                return <EditTextNote note={note} />;
            }
          })
          .get()}

        <footer className="note-editor__footer">
          <button onClick={this.handleDone}>Done</button>
        </footer>
      </div>
    );
  }
}
