"""pytest : les modules de tools/dsgen s'importent depuis ce dossier (comme quand gen_ds.py s'exécute)."""
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
