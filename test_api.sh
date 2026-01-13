#!/bin/bash

# Color output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

PASS=0
FAIL=0
BASE_URL="http://127.0.0.1:3000"

# Generate unique values to avoid conflicts on repeated runs
TIMESTAMP=$(date +%s%N)
REPO_NAME="test-repo-$TIMESTAMP"
USER_NAME="testuser$TIMESTAMP"
ORG_NAME="testorg$TIMESTAMP"
PKG_NAME="testpkg$TIMESTAMP"

test_endpoint() {
    local method=$1
    local endpoint=$2
    local data=$3
    local expected_code=$4
    local description=$5

    echo -n "Testing $method $endpoint... "
    
    if [ -z "$data" ]; then
        response=$(curl -s -w "\n%{http_code}" -X $method "$BASE_URL$endpoint")
    else
        response=$(curl -s -w "\n%{http_code}" -X $method "$BASE_URL$endpoint" -H "Content-Type: application/json" -d "$data")
    fi
    
    http_code=$(echo "$response" | tail -n1)
    body=$(echo "$response" | head -n-1)
    
    if [ "$http_code" == "$expected_code" ]; then
        echo -e "${GREEN}✓ OK${NC} (HTTP $http_code)"
        ((PASS++))
    else
        echo -e "${RED}✗ FAIL${NC} (Expected $expected_code, got $http_code)"
        echo "  Response: $body"
        ((FAIL++))
    fi
}

echo "======================================="
echo "Codeza API Test Suite"
echo "======================================="
echo ""

echo "=== Repository Endpoints ==="
test_endpoint "GET" "/api/v1/repos" "" "200" "List repos"
test_endpoint "GET" "/api/v1/repos/admin/codeza" "" "200" "Get repo"
test_endpoint "POST" "/api/v1/user/repos" "{\"name\":\"$REPO_NAME\",\"description\":\"Test\",\"private\":false,\"auto_init\":false}" "201" "Create repo"

echo ""
echo "=== Issue Endpoints ==="
test_endpoint "GET" "/api/v1/repos/admin/codeza/issues" "" "200" "List issues"
test_endpoint "GET" "/api/v1/repos/admin/codeza/issues/1" "" "200" "Get issue"
test_endpoint "POST" "/api/v1/repos/admin/codeza/issues" '{"title":"Test Issue","body":"Test"}' "201" "Create issue"

echo ""
echo "=== Pull Request Endpoints ==="
test_endpoint "GET" "/api/v1/repos/admin/codeza/pulls" "" "200" "List PRs"
test_endpoint "POST" "/api/v1/repos/admin/codeza/pulls" '{"title":"Test PR","body":"Test","base":"main","head":"feature"}' "201" "Create PR"

echo ""
echo "=== Release Endpoints ==="
test_endpoint "GET" "/api/v1/repos/admin/codeza/releases" "" "200" "List releases"
test_endpoint "POST" "/api/v1/repos/admin/codeza/releases" '{"tag_name":"v1.1.0","name":"Release","body":"Test","draft":false,"prerelease":false}' "201" "Create release"

echo ""
echo "=== Label Endpoints ==="
test_endpoint "GET" "/api/v1/repos/admin/codeza/labels" "" "200" "List labels"
test_endpoint "POST" "/api/v1/repos/admin/codeza/labels" '{"name":"feature","color":"#00FF00"}' "201" "Create label"

echo ""
echo "=== Milestone Endpoints ==="
test_endpoint "GET" "/api/v1/repos/admin/codeza/milestones" "" "200" "List milestones"
test_endpoint "POST" "/api/v1/repos/admin/codeza/milestones" '{"title":"v1.1"}' "201" "Create milestone"
test_endpoint "GET" "/api/v1/repos/admin/codeza/milestones/1" "" "200" "Get milestone"

echo ""
echo "=== User Endpoints ==="
test_endpoint "GET" "/api/v1/users/admin" "" "200" "Get user"
test_endpoint "POST" "/api/v1/users/login" '{"username":"admin","password":"password"}' "200" "Login user"
test_endpoint "POST" "/api/v1/users/register" "{\"username\":\"$USER_NAME\",\"email\":\"${USER_NAME}@test.com\",\"password\":\"pass123\"}" "201" "Register user"

echo ""
echo "=== Comment Endpoints ==="
test_endpoint "GET" "/api/v1/repos/admin/codeza/issues/1/comments" "" "200" "List comments"
test_endpoint "POST" "/api/v1/repos/admin/codeza/issues/1/comments" '{"body":"Great work!"}' "201" "Create comment"

echo ""
echo "=== Star/Watch/Fork Endpoints ==="
test_endpoint "POST" "/api/v1/repos/admin/codeza/star" "" "204" "Star repo"
test_endpoint "POST" "/api/v1/repos/admin/codeza/watch" "" "204" "Watch repo"
test_endpoint "POST" "/api/v1/repos/admin/codeza/fork" "" "200" "Fork repo"

echo ""
echo "=== Topic Endpoints ==="
test_endpoint "GET" "/api/v1/repos/admin/codeza/topics" "" "200" "List topics"
test_endpoint "PUT" "/api/v1/repos/admin/codeza/topics" '{"topics":["rust","axum"]}' "204" "Update topics"

echo ""
echo "=== Hook/Webhook Endpoints ==="
test_endpoint "GET" "/api/v1/repos/admin/codeza/hooks" "" "200" "List webhooks"
test_endpoint "POST" "/api/v1/repos/admin/codeza/hooks" '{"url":"http://example.com/hook","events":["push"],"active":true}' "201" "Create webhook"
test_endpoint "GET" "/api/v1/repos/admin/codeza/hooks/1/deliveries" "" "200" "List webhook deliveries"

echo ""
echo "=== Organization Endpoints ==="
test_endpoint "POST" "/api/v1/orgs" "{\"username\":\"$ORG_NAME\",\"description\":\"Test Org\"}" "201" "Create org"
test_endpoint "GET" "/api/v1/orgs/codeza-org" "" "200" "Get org"
test_endpoint "GET" "/api/v1/orgs/codeza-org/repos" "" "200" "List org repos"
test_endpoint "GET" "/api/v1/orgs/codeza-org/teams" "" "200" "List org teams"
test_endpoint "GET" "/api/v1/orgs/codeza-org/members" "" "200" "List org members"

echo ""
echo "=== Search Endpoints ==="
test_endpoint "GET" "/api/v1/repos/search?q=test" "" "200" "Search repos"
test_endpoint "GET" "/api/v1/repos/admin/codeza/search?q=main" "" "200" "Search code"

echo ""
echo "=== Key/SSH Endpoints ==="
test_endpoint "GET" "/api/v1/user/keys" "" "200" "List SSH keys"
test_endpoint "POST" "/api/v1/user/keys" '{"title":"My Key","key":"ssh-rsa AAA..."}' "201" "Create SSH key"

echo ""
echo "=== GPG Key Endpoints ==="
test_endpoint "GET" "/api/v1/user/gpg_keys" "" "200" "List GPG keys"
test_endpoint "POST" "/api/v1/user/gpg_keys" '{"armored_public_key":"-----BEGIN PGP PUBLIC KEY BLOCK-----\nVersion: GnuPG v1\n\nmQENBFxYUZIBCADc..."}' "201" "Create GPG key"

echo ""
echo "=== Package Endpoints ==="
test_endpoint "GET" "/api/v1/packages/admin" "" "200" "List packages"
test_endpoint "GET" "/api/v1/packages/admin/cargo/my-lib/1.0.0" "" "200" "Get package"
test_endpoint "POST" "/api/v1/packages/admin" "{\"name\":\"$PKG_NAME\",\"version\":\"1.0.0\",\"package_type\":\"cargo\"}" "201" "Upload package"

echo ""
echo "=== Branch Endpoints ==="
test_endpoint "GET" "/api/v1/repos/admin/codeza/branches" "" "200" "List branches"
test_endpoint "POST" "/api/v1/repos/admin/codeza/branches" '{"name":"feature/test","base":"main"}' "201" "Create branch"

echo ""
echo "=== Commit Endpoints ==="
test_endpoint "GET" "/api/v1/repos/admin/codeza/commits" "" "200" "List commits"
test_endpoint "GET" "/api/v1/repos/admin/codeza/commits/abc123456789/diff" "" "200" "Get commit diff"

echo ""
echo "=== Notification Endpoints ==="
test_endpoint "GET" "/api/v1/notifications" "" "200" "List notifications"
test_endpoint "PATCH" "/api/v1/notifications/threads/1" "" "204" "Mark as read"

echo ""
echo "=== Setting Endpoints ==="
test_endpoint "GET" "/api/v1/repos/admin/codeza/settings" "" "200" "Get repo settings"
test_endpoint "PATCH" "/api/v1/repos/admin/codeza/settings" '{"website":"https://example.com"}' "204" "Update repo settings"
test_endpoint "GET" "/api/v1/user/settings" "" "200" "Get user settings"
test_endpoint "PATCH" "/api/v1/user/settings" '{"theme":"dark"}' "204" "Update user settings"

echo ""
echo "=== Project Endpoints ==="
test_endpoint "GET" "/api/v1/repos/admin/codeza/projects" "" "200" "List projects"

echo ""
echo "=== Admin Endpoints ==="
test_endpoint "GET" "/api/v1/admin/stats" "" "200" "Get admin stats"
test_endpoint "GET" "/api/v1/admin/users" "" "200" "Admin list users"
test_endpoint "POST" "/api/v1/admin/users" "{\"username\":\"admin$TIMESTAMP\",\"email\":\"admin${TIMESTAMP}@test.com\",\"password\":\"pass123\"}" "201" "Admin create user"

echo ""
echo "======================================="
echo "Test Results"
echo "======================================="
echo -e "${GREEN}Passed: $PASS${NC}"
echo -e "${RED}Failed: $FAIL${NC}"
echo "Total: $((PASS + FAIL))"
echo "======================================="

if [ $FAIL -gt 0 ]; then
    exit 1
fi
