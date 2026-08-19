import type { Icon } from './icon';

export type User = {
  id: string;
  inbox: string;
  preferredUsername: string;
  publicKey: {
    id: string;
    owner: string;
    publicKeyPem: string;
  };
  name?: string;
  type: string;
  icon?: Icon;
};
