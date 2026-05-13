#!/bin/bash
# Manual comment posting for FUL-25 when API restores
# Run when http://100.83.52.32:3100 is reachable

API_URL="${PAPERCLIP_API_URL:-http://100.83.52.32:3100}"
API_KEY="${PAPERCLIP_API_KEY:-}"

if [ -z "$API_KEY" ]; then
  echo "ERROR: PAPERCLIP_API_KEY not set"
  exit 1
fi

curl -X POST \
  -H "Authorization: Bearer $API_KEY" \
  -H "Content-Type: application/json" \
  -H "X-Paperclip-Run-Id: manual-cto-ful25-done" \
  -d '{
    "comment": "## FUL-25: Build Complete — CTO Verification\n\nVerified work products on disk:\n\n- **Build**: dist/ with 163KB JS bundle, 6KB CSS, 0.4KB HTML entry\n- **Screenshot**: FUL-25-demo-screenshot.png (39KB, 1280x720 PNG)\n- **Build time**: 994ms via Vite\n\nMarking issue done. Deploy dist/ to static hosting (Vercel/Netlify/GH Pages) at your discretion."
  }' \
  "${API_URL}/api/issues/FUL-25/comments"

echo ""
echo "Now PATCH issue to done..."
curl -X PATCH \
  -H "Authorization: Bearer $API_KEY" \
  -H "Content-Type: application/json" \
  -H "X-Paperclip-Run-Id: manual-cto-ful25-done" \
  -d '{"status": "done"}' \
  "${API_URL}/api/issues/FUL-25"