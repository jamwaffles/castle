import * as React from "react";
import { AnyNote, TextNote } from "../../types";
import { Option } from 'funfix';

export interface EditChecklistNoteProps {
  onChange: (content: string) => any;
  note: TextNote;
}

export default class EditTextNote extends React.PureComponent<
  EditChecklistNoteProps,
  any
> {
  handleChange = (e: any) => {
    const content = e.target.value;

    this.props.onChange(content);
  }

  render() {
    const { content } = this.props.note;

    return (
      <div className="edit-text-note">
        <textarea
          onChange={this.handleChange}
          value={content}
        />
      </div>
    );
  }
}
