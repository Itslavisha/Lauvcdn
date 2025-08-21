export type UploadResult = { fileId: string }

export async function upload(_file: File): Promise<UploadResult> {
  return { fileId: "todo" }
}

export async function getFile(_id: string): Promise<Uint8Array> {
  return new Uint8Array()
}

export async function recharge(_amount: number): Promise<void> {}
