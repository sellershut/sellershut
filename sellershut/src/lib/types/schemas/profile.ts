import { z } from 'zod';

export const profileSchema = z.object({
  username: z
    .string()
    .min(3, 'Username must be at least 3 characters')
    .max(15, 'Username must be at most 15 characters')
    .regex(/^[a-z0-9_]+$/, 'Username can only contain lowercase letters, numbers, and underscores'),
  displayName: z.string().max(50, 'Display name must be 50 characters or fewer').optional(),
  description: z.string().max(300, 'Description must be 300 characters or fewer').optional(),
});

export type Profile = z.infer<typeof profileSchema>;
