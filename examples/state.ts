// Local state will not be persisted after recompile
const localState = { count: 0 };

// State will be persisted after recompile
const state = createState({
  count: 0,
});

export function update() {
  if (frameCount() % 60 == 0) {
    localState.count += 1;
    state.count += 1;

    console.log(`Local state count: ${localState.count}`);
    console.log(`State count: ${state.count}`);
  }
}
