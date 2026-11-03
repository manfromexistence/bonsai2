"use client";

import Link from "next/link";
import { LogOut } from "lucide-react";
import { Button } from "@/components/ui/button";
import { leavePortal } from "@/interfaces/actions/portal";
import { useLocale } from "@/lib/i18n";

export function PortalShell({
  title,
  subtitle,
  role,
  children,
  aside,
}: {
  title: string;
  subtitle: string;
  role: string;
  children: React.ReactNode;
  aside?: React.ReactNode;
}) {
  const { dict } = useLocale();
  return (
    <div className="min-h-screen bg-paper">
      <header className="border-b border-khaki/30 bg-green text-paper">
        <div className="mx-auto flex max-w-[1240px] flex-wrap items-center gap-6 px-5 py-6 sm:px-8">
          <img
            src="/logo.png"
            alt="Al Quran Learning Institute"
            className="h-10 w-10 rounded-full object-cover ring-1 ring-gold/40"
          />
          <div>
            <p className="eyebrow text-gold-light">
              {role} · {dict.portal.shell.portalSuffix}
            </p>
            <h1 className="mt-1 font-display text-[1.65rem] font-semibold leading-none">
              {title}
            </h1>
            <p className="mt-2 text-[0.9rem] text-paper/70">{subtitle}</p>
          </div>
          <div className="ms-auto flex flex-wrap items-center gap-3">
            {aside}
            <Button asChild variant="ghostLight" size="sm">
              <Link href="/">{dict.portal.shell.publicSite}</Link>
            </Button>
            <form action={leavePortal}>
              <Button variant="ghostLight" size="sm" type="submit">
                <LogOut className="h-3.5 w-3.5 rtl:-scale-x-100" />{" "}
                {dict.portal.shell.logout}
              </Button>
            </form>
          </div>
        </div>
      </header>
      <div className="mx-auto max-w-[1240px] px-5 py-10 sm:px-8">
        {children}
      </div>
    </div>
  );
}

export function StatCard({
  label,
  value,
  note,
}: {
  label: string;
  value: string;
  note?: string;
}) {
  return (
    <div className="border border-khaki/30 bg-sheet p-6 transition-colors hover:border-gold/50">
      <p className="eyebrow text-khaki">{label}</p>
      <p className="mt-3 font-display text-[2.15rem] font-bold leading-none text-green tabular">
        {value}
      </p>
      {note ? <p className="mt-2.5 text-[0.82rem] text-khaki">{note}</p> : null}
    </div>
  );
}
