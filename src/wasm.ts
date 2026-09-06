import init, {
    BinarySearch,
    DebugInfo,
    RailSimulation,
    SearchStatus,
} from "../wasm/pkg";

export { init, BinarySearch, DebugInfo, RailSimulation, SearchStatus };

export async function initWasm(): Promise<void> {
    await init();
}
