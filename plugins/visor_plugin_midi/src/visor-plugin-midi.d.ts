// deno-lint-ignore no-var
declare var midi: {
  listInputDevices(): Array<string>;
  connectInputDevice(name: string): void;
  disconnectInputDevice(name: string): void;
  loadMapping(path: string): void;
  clearMapping(): void;
  note(id: string): Note;
  encoder(id: string): Encoder;
  control(id: string): number;
};

interface Encoder {
  increment(): boolean;
  decrement(): boolean;
}

interface Note {
  on(): boolean;
  off(): boolean;
  down(): boolean;
  velocity(): number;
}
