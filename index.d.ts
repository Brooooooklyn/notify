export interface NotifyEvent {}

export function watch(dir: string, cb: (err: Error | null, event: NotifyEvent) => void): () => void
