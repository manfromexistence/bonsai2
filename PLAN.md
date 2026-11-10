# PLAN — Admin Role System

Single plan: replace the env allow-list (`ADMIN_CLERK_USER_IDS`) with a DB-backed
admin role system so the super admin can assign admins from members.

## Super admin
- `essencefromexistence` (Clerk user) — full control, can promote/demote.

## Default admins (seeded on first provision)
- ajju40959@gmail.com
- shohanmomtaz@gmail.com
- essencefromexistence@gmail.com

## Work
1. Add `admins` table (`user_id`, `role`, `granted_by`, `granted_at`).
2. `requireAdmin()` checks the table (super admin always passes).
3. Admin console: list members, promote/demote to admin.
4. Seed the three default admins on provision.
5. Clerk: membership optional + Personal Account enabled (done in dashboard).

Done-when: signing in as one of the three emails reaches /admin; super admin can
promote a member and they gain access on next request.

---

MIT OR Apache-2.0
updated: 2026-11-10 00:00 UTC