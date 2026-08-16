export type NavUser = {
  id: string;
  username: string;
  displayName: string;
  handle: string;
  avatarUrl: string | null;

  instance: {
    domain: string;
    name?: string;
    url?: string;
  };
};
