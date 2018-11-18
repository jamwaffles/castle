export type IsoDate = string;
export type Uuid = string;

export interface Note {
  id: Uuid;
  title: string;
  content: string;
  created_at: IsoDate;
}

export interface NewNote {
  title?: string;
  content?: string;
}

export type AnyNote = Note | NewNote;
