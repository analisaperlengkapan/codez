import os
import time
from playwright.sync_api import sync_playwright

BASE_URL = "http://127.0.0.1:3000"
REPO_SCREENSHOT_DIR = "screenshots"

ROUTES = [
    ("01_dashboard", "/"),
    ("02_explore", "/explore"),
    ("03_repo_detail", "/repos/admin/codeza"),
    ("04_issues", "/repos/admin/codeza/issues"),
    ("05_pulls", "/repos/admin/codeza/pulls"),
    ("06_actions", "/repos/admin/codeza/actions"),
    ("07_security", "/repos/admin/codeza/security"),
    ("08_pulse", "/repos/admin/codeza/pulse"),
    ("09_discussions", "/repos/admin/codeza/discussions"),
    ("10_projects", "/repos/admin/codeza/projects"),
    ("11_wiki", "/repos/admin/codeza/wiki"),
    ("12_packages", "/packages/admin"),
    ("13_admin", "/admin"),
    ("14_settings", "/settings/profile"),
]

def capture_repo_screenshots():
    os.makedirs(REPO_SCREENSHOT_DIR, exist_ok=True)
    os.makedirs("/home/jules/verification/videos", exist_ok=True)

    with sync_playwright() as p:
        browser = p.chromium.launch(headless=True)
        context = browser.new_context(
            record_video_dir="/home/jules/verification/videos"
        )
        page = context.new_page()

        try:
            for name, path in ROUTES:
                url = f"{BASE_URL}{path}"
                print(f"Navigating to {url}...")
                page.goto(url, wait_until="networkidle")
                page.wait_for_timeout(1000)

                filepath = os.path.join(REPO_SCREENSHOT_DIR, f"{name}.png")
                page.screenshot(path=filepath, full_page=True)
                print(f"Saved screenshot to repo: {filepath}")

        finally:
            context.close()
            browser.close()

if __name__ == "__main__":
    capture_repo_screenshots()
