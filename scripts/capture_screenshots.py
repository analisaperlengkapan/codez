"""Capture full-page screenshots of every Codeza UI route.

Usage:
    python scripts/capture_screenshots.py [output_dir]

Requires a running backend serving the built frontend at
``http://127.0.0.1:3000`` (see README) and the Playwright browsers installed.
"""

import os
import sys

from playwright.sync_api import sync_playwright

BASE_URL = os.environ.get("CODEZA_BASE_URL", "http://127.0.0.1:3000")

ROUTES = [
    ("01_dashboard", "/"),
    ("02_explore", "/explore"),
    ("03_repo_detail", "/repos/admin/codeza"),
    ("04_repo_code", "/repos/admin/codeza/src/src/main.rs"),
    ("05_commits", "/repos/admin/codeza/commits"),
    ("06_branches", "/repos/admin/codeza/branches"),
    ("07_tags", "/repos/admin/codeza/tags"),
    ("08_issues", "/repos/admin/codeza/issues"),
    ("09_issue_detail", "/repos/admin/codeza/issues/1"),
    ("10_pulls", "/repos/admin/codeza/pulls"),
    ("11_pull_detail", "/repos/admin/codeza/pulls/1"),
    ("12_labels", "/repos/admin/codeza/labels"),
    ("13_milestones", "/repos/admin/codeza/milestones"),
    ("14_releases", "/repos/admin/codeza/releases"),
    ("15_actions", "/repos/admin/codeza/actions"),
    ("16_security", "/repos/admin/codeza/security"),
    ("17_pulse", "/repos/admin/codeza/pulse"),
    ("18_discussions", "/repos/admin/codeza/discussions"),
    ("19_projects", "/repos/admin/codeza/projects"),
    ("20_wiki", "/repos/admin/codeza/wiki"),
    ("21_collaborators", "/repos/admin/codeza/collaborators"),
    ("22_repo_settings", "/repos/admin/codeza/settings"),
    ("23_packages", "/packages/admin"),
    ("24_notifications", "/notifications"),
    ("25_search", "/search"),
    ("26_admin", "/admin"),
    ("27_admin_users", "/admin/users"),
    ("28_user_profile", "/users/admin"),
    ("29_org_profile", "/orgs/admin"),
    ("30_settings", "/settings/profile"),
    ("31_login", "/login"),
    ("32_register", "/register"),
    ("33_create_repo", "/repo/create"),
    ("34_compare", "/repos/admin/codeza/compare"),
    ("35_code_search", "/repos/admin/codeza/search"),
    ("36_wiki_edit", "/repos/admin/codeza/wiki/pages/Home/edit"),
    ("37_milestone_detail", "/repos/admin/codeza/milestones/1"),
    ("38_release_detail", "/repos/admin/codeza/releases/1"),
    ("39_workflow_runs", "/repos/admin/codeza/actions/workflows/1"),
    ("40_settings_branches", "/repos/admin/codeza/settings/branches"),
    ("41_settings_webhooks", "/repos/admin/codeza/settings/webhooks"),
    ("42_settings_secrets", "/repos/admin/codeza/settings/secrets"),
    ("43_settings_keys", "/repos/admin/codeza/settings/keys"),
    ("44_settings_lfs", "/repos/admin/codeza/settings/lfs"),
    ("45_org_create", "/org/create"),
    ("46_migrate_repo", "/repo/migrate"),
    ("47_user_followers", "/users/admin/followers"),
]


def capture(output_dir: str) -> None:
    os.makedirs(output_dir, exist_ok=True)
    with sync_playwright() as p:
        browser = p.chromium.launch(headless=True)
        page = browser.new_context(viewport={"width": 1440, "height": 1000}).new_page()
        for name, path in ROUTES:
            try:
                page.goto(BASE_URL + path, wait_until="networkidle", timeout=20000)
            except Exception as exc:  # noqa: BLE001 - report and keep capturing
                print(f"navigation error for {path}: {exc}")
            page.wait_for_timeout(600)
            page.screenshot(path=os.path.join(output_dir, f"{name}.png"), full_page=True)
            print(f"saved {name}")
        browser.close()


if __name__ == "__main__":
    capture(sys.argv[1] if len(sys.argv) > 1 else "docs/screenshots")

