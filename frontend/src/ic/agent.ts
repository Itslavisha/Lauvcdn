import { HttpAgent, Actor } from '@dfinity/agent'
import { AuthClient } from '@dfinity/auth-client'

export async function getIdentity() {
  const auth = await AuthClient.create()
  if (!(await auth.isAuthenticated())) {
    await auth.login({ 
      identityProvider: process.env.NODE_ENV === 'development' 
        ? `http://127.0.0.1:4943/?canisterId=${process.env.INTERNET_IDENTITY_CANISTER_ID}`
        : 'https://identity.ic0.app'
    })
  }
  return auth.getIdentity()
}

export async function getAgent(authenticated = false) {
  const identity = authenticated ? await getIdentity() : undefined
  const host = import.meta.env.VITE_HOST || 'http://127.0.0.1:4943'
  const agent = new HttpAgent({ identity, host })
  
  if (import.meta.env.VITE_DFX_NETWORK === 'local') {
    await agent.fetchRootKey()
  }
  return agent
}

export async function getActor(canisterId: string, idlFactory: any, authenticated = false) {
  const agent = await getAgent(authenticated)
  return Actor.createActor(idlFactory, { agent, canisterId }) as any
}
