export const navItems = [
  { key: 'marketplace', label: 'Marketplace' },
  { key: 'discover', label: 'Discover' },
  { key: 'selling', label: 'Selling' },
  { key: 'network', label: 'Network' },
] as const;

export type MenuKey = (typeof navItems)[number]['key'];

export type NavLink = {
  label: string;
  href: string;
  description?: string;
  external?: boolean;
};

export type MenuSection = {
  heading: string;
  primary: NavLink[];
  secondary: NavLink[];
};

export const menus: Record<MenuKey, MenuSection> = {
  marketplace: {
    heading: 'Marketplace',

    primary: [
      {
        label: 'Browse all',
        href: '/marketplace',
        description: 'Explore listings across the network',
      },
      {
        label: 'Local',
        href: '/marketplace/local',
        description: 'Listings from your home instance',
      },
      {
        label: 'Federated',
        href: '/marketplace/federated',
        description: 'Listings from connected marketplaces',
      },
      {
        label: 'Categories',
        href: '/categories',
        description: 'Browse listings by category',
      },
    ],

    secondary: [
      {
        label: 'Recently added',
        href: '/marketplace/recent',
      },
      {
        label: 'Popular listings',
        href: '/marketplace/popular',
      },
      {
        label: 'Saved listings',
        href: '/saved',
      },
    ],
  },

  discover: {
    heading: 'Discover',

    primary: [
      {
        label: 'Featured',
        href: '/discover',
        description: 'Interesting listings from across the network',
      },
      {
        label: 'Sellers',
        href: '/sellers',
        description: 'Discover sellers across the federation',
      },
      {
        label: 'Servers',
        href: '/servers',
        description: 'Explore connected marketplace instances',
      },
      {
        label: 'Nearby',
        href: '/discover/nearby',
        description: 'Find listings closer to you',
      },
    ],

    secondary: [
      {
        label: 'New sellers',
        href: '/discover/sellers',
      },
      {
        label: 'Recently federated',
        href: '/discover/federated',
      },
      {
        label: 'Browse categories',
        href: '/categories',
      },
    ],
  },

  selling: {
    heading: 'Selling',

    primary: [
      {
        label: 'Create a listing',
        href: '/sell',
        description: 'List something on the marketplace',
      },
      {
        label: 'My listings',
        href: '/account/listings',
        description: 'Manage active, sold and archived listings',
      },
      {
        label: 'Messages',
        href: '/messages',
        description: 'Talk directly with buyers and sellers',
      },
    ],

    secondary: [
      {
        label: 'Selling guide',
        href: '/help/selling',
      },
      {
        label: 'Safety',
        href: '/safety',
      },
      {
        label: 'Marketplace rules',
        href: '/rules',
      },
    ],
  },

  network: {
    heading: 'Federation',

    primary: [
      {
        label: 'Explore servers',
        href: '/servers',
        description: 'Discover marketplaces across the network',
      },
      {
        label: 'How federation works',
        href: '/about/federation',
        description: 'Learn how listings move between servers',
      },
      {
        label: 'Network status',
        href: '/status',
        description: 'Check federation and service availability',
      },
    ],

    secondary: [
      {
        label: 'Trust & safety',
        href: '/safety',
      },
      {
        label: 'Moderation',
        href: '/moderation',
      },
      {
        label: 'Developers',
        href: '/developers',
      },
    ],
  },
};

export const quickSearchLinks = [
  {
    label: 'Browse all listings',
    href: '/marketplace',
  },
  {
    label: 'Browse categories',
    href: '/categories',
  },
  {
    label: 'Find sellers',
    href: '/sellers',
  },
  {
    label: 'Explore servers',
    href: '/servers',
  },
];

export const mobileLinks = [
  {
    label: 'Marketplace',
    href: '/marketplace',
  },
  {
    label: 'Local',
    href: '/marketplace/local',
  },
  {
    label: 'Federated',
    href: '/marketplace/federated',
  },
  {
    label: 'Discover',
    href: '/discover',
  },
  {
    label: 'Categories',
    href: '/categories',
  },
  {
    label: 'Sell an item',
    href: '/sell',
  },
  {
    label: 'Servers',
    href: '/servers',
  },
];
