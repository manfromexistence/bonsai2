"use client";

import * as React from "react";
import { DashboardHeader } from "@/components/admin/dashboard-header";
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
    <div className="flex min-h-screen flex-col bg-paper">
      {/* Unified Universal Dashboard Header */}
      <DashboardHeader currentPageTitle={title} />

      {/* Main Content Area */}
      <main className="mx-auto w-full max-w-[1360px] p-5 sm:p-8">
        {/* Folio Hero Banner */}
        <div className="relative mb-8 overflow-hidden border border-khaki/30 bg-green p-6 text-paper shadow-sm sm:p-8">
          <div className="flex flex-wrap items-center justify-between gap-5">
            <div>
              <div className="flex items-center gap-2">
                <span className="eyebrow text-gold-light">
                  {role.toUpperCase()} WORKSPACE
                </span>
                <span className="h-1 w-1 rounded-full bg-gold" />
                <span className="eyebrow text-paper/70">
                  {dict.portal.shell.portalSuffix}
                </span>
              </div>
              <h1 className="mt-2 font-display text-[clamp(1.75rem,3.2vw,2.5rem)] font-bold text-paper">
                {title}
              </h1>
              <p className="mt-2 max-w-2xl text-sm leading-relaxed text-paper/80">
                {subtitle}
              </p>
            </div>
            {aside ? (
              <div className="flex flex-wrap items-center gap-2.5">
                {aside}
              </div>
            ) : null}
          </div>
        </div>

        {children}
      </main>
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
    <div className="border border-khaki/30 bg-sheet p-5 transition-colors hover:border-gold/50 shadow-xs">
      <p className="eyebrow text-khaki">{label}</p>
      <p className="mt-2.5 font-display text-3xl font-bold leading-none text-green tabular-nums">
        {value}
      </p>
      {note ? <p className="mt-2 text-xs text-ink-soft">{note}</p> : null}
    </div>
  );
}
